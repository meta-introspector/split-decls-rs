macro_rules! deps {
    () => {
        ConsiderDereferencing!();
        NonConstAwait!();
        NonConstFmtMacroCall!();
        NonConstMatchEq!();
        NonConstQuestionBranch!();
        NonConstQuestionFromResidual!();
        NonConstOperator!();
        NonConstFnCall!();
        NonConstClosureNote!();
        NonConstClosure!();
        NonConstTryBlockFromOutput!();
        NonConstDerefCoercion!();
        ConstCx!();
        NonConstForLoopIntoIter!();
    };
}

macro_rules! build_error_for_const_call {
    () => {
        deps!();
        # [doc = " Build an error message reporting that a function call is not const (or only"] # [doc = " conditionally const). In case that this call is desugared (like an operator"] # [doc = " or sugar from something like a `for` loop), try to build a better error message"] # [doc = " that doesn't call it a method."] fn build_error_for_const_call < 'tcx > (ccx : & ConstCx < '_ , 'tcx > , callee : DefId , args : ty :: GenericArgsRef < 'tcx > , span : Span , call_source : CallSource , non_or_conditionally : & 'static str , note_trait_if_possible : impl FnOnce (& mut Diag < 'tcx > , Ty < 'tcx > , DefId) ,) -> Diag < 'tcx > { let tcx = ccx . tcx ; let call_kind = call_kind (tcx , ccx . typing_env , callee , args , span , call_source . from_hir_call () , None) ; debug ! (? call_kind) ; let mut err = match call_kind { CallKind :: Normal { desugaring : Some ((kind , self_ty)) , .. } => { macro_rules ! error { ($ err : ident) => { tcx . dcx () . create_err (errors ::$ err { span , ty : self_ty , kind : ccx . const_kind () , non_or_conditionally , }) } ; } match kind { CallDesugaringKind :: ForLoopIntoIter | CallDesugaringKind :: ForLoopNext => { error ! (NonConstForLoopIntoIter) } CallDesugaringKind :: QuestionBranch => { error ! (NonConstQuestionBranch) } CallDesugaringKind :: QuestionFromResidual => { error ! (NonConstQuestionFromResidual) } CallDesugaringKind :: TryBlockFromOutput => { error ! (NonConstTryBlockFromOutput) } CallDesugaringKind :: Await => { error ! (NonConstAwait) } } } CallKind :: FnCall { fn_trait_id , self_ty } => { let note = match self_ty . kind () { FnDef (def_id , ..) => { let span = tcx . def_span (* def_id) ; if ccx . tcx . is_const_fn (* def_id) { span_bug ! (span , "calling const FnDef errored when it shouldn't") ; } Some (errors :: NonConstClosureNote :: FnDef { span }) } FnPtr (..) => Some (errors :: NonConstClosureNote :: FnPtr) , Closure (..) => Some (errors :: NonConstClosureNote :: Closure) , _ => None , } ; let mut err = tcx . dcx () . create_err (errors :: NonConstClosure { span , kind : ccx . const_kind () , note , non_or_conditionally , }) ; note_trait_if_possible (& mut err , self_ty , fn_trait_id) ; err } CallKind :: Operator { trait_id , self_ty , .. } => { let mut err = if let CallSource :: MatchCmp = call_source { tcx . dcx () . create_err (errors :: NonConstMatchEq { span , kind : ccx . const_kind () , ty : self_ty , non_or_conditionally , }) } else { let mut sugg = None ; if ccx . tcx . is_lang_item (trait_id , LangItem :: PartialEq) { match (args [0] . kind () , args [1] . kind ()) { (GenericArgKind :: Type (self_ty) , GenericArgKind :: Type (rhs_ty)) if self_ty == rhs_ty && self_ty . is_ref () && self_ty . peel_refs () . is_primitive () => { let mut num_refs = 0 ; let mut tmp_ty = self_ty ; while let rustc_middle :: ty :: Ref (_ , inner_ty , _) = tmp_ty . kind () { num_refs += 1 ; tmp_ty = * inner_ty ; } let deref = "*" . repeat (num_refs) ; if let Ok (call_str) = ccx . tcx . sess . source_map () . span_to_snippet (span) && let Some (eq_idx) = call_str . find ("==") && let Some (rhs_idx) = call_str [(eq_idx + 2) ..] . find (| c : char | ! c . is_whitespace ()) { let rhs_pos = span . lo () + BytePos :: from_usize (eq_idx + 2 + rhs_idx) ; let rhs_span = span . with_lo (rhs_pos) . with_hi (rhs_pos) ; sugg = Some (errors :: ConsiderDereferencing { deref , span : span . shrink_to_lo () , rhs_span , }) ; } } _ => { } } } tcx . dcx () . create_err (errors :: NonConstOperator { span , kind : ccx . const_kind () , sugg , non_or_conditionally , }) } ; note_trait_if_possible (& mut err , self_ty , trait_id) ; err } CallKind :: DerefCoercion { deref_target_span , deref_target_ty , self_ty } => { let target = if let Some (deref_target_span) = deref_target_span && tcx . sess . source_map () . is_span_accessible (deref_target_span) { Some (deref_target_span) } else { None } ; let mut err = tcx . dcx () . create_err (errors :: NonConstDerefCoercion { span , ty : self_ty , kind : ccx . const_kind () , target_ty : deref_target_ty , deref_target : target , non_or_conditionally , }) ; note_trait_if_possible (& mut err , self_ty , tcx . require_lang_item (LangItem :: Deref , span)) ; err } _ if tcx . opt_parent (callee) == tcx . get_diagnostic_item (sym :: FmtArgumentsNew) => { ccx . dcx () . create_err (errors :: NonConstFmtMacroCall { span , kind : ccx . const_kind () , non_or_conditionally , }) } _ => ccx . dcx () . create_err (errors :: NonConstFnCall { span , def_descr : ccx . tcx . def_descr (callee) , def_path_str : ccx . tcx . def_path_str_with_args (callee , args) , kind : ccx . const_kind () , non_or_conditionally , }) , } ; err . note (format ! ("calls in {}s are limited to constant functions, \
             tuple structs and tuple variants" , ccx . const_kind () ,)) ; err }
    };
}

build_error_for_const_call!()