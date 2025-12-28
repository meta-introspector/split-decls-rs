macro_rules! deps {
    () => {
        NakedFunctionsAsmBlock!();
        ItemKind!();
        CheckInlineAssembly!();
        NakedFunctionsMustNakedAsm!();
    };
}

macro_rules! check_asm {
    () => {
        deps!();
        # [doc = " Checks that function body contains a single inline assembly block."] fn check_asm < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , body : & 'tcx hir :: Body < 'tcx >) { let mut this = CheckInlineAssembly { items : Vec :: new () } ; this . visit_body (body) ; if let [(ItemKind :: NakedAsm | ItemKind :: Err , _)] = this . items [..] { } else { let mut must_show_error = false ; let mut has_naked_asm = false ; let mut has_err = false ; let mut multiple_asms = vec ! [] ; let mut non_asms = vec ! [] ; for & (kind , span) in & this . items { match kind { ItemKind :: NakedAsm if has_naked_asm => { must_show_error = true ; multiple_asms . push (span) ; } ItemKind :: NakedAsm => has_naked_asm = true , ItemKind :: InlineAsm => { has_err = true ; tcx . dcx () . emit_err (NakedFunctionsMustNakedAsm { span }) ; } ItemKind :: NonAsm => { must_show_error = true ; non_asms . push (span) ; } ItemKind :: Err => has_err = true , } } if must_show_error || ! has_err { tcx . dcx () . emit_err (NakedFunctionsAsmBlock { span : tcx . def_span (def_id) , multiple_asms , non_asms , }) ; } } }
    };
}

check_asm!();