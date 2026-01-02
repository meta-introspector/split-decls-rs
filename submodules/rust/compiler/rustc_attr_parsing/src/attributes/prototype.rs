mkuse!{use rustc_feature :: { AttributeTemplate , template } ;}
mkuse!{use rustc_hir :: Target ;}
mkuse!{use rustc_hir :: attrs :: { AttributeKind , MirDialect , MirPhase } ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use super :: { AttributeOrder , OnDuplicate } ;}
mkuse!{use crate :: attributes :: SingleAttributeParser ;}
mkuse!{use crate :: context :: { AcceptContext , Stage } ;}
mkuse!{use crate :: parser :: ArgParser ;}
mkuse!{use crate :: target_checking :: AllowedTargets ;}
mkuse!{use crate :: target_checking :: Policy :: Allow ;}
mkitem!{mkstruct!{pub (crate) struct CustomMirParser ;}}
mkitem!{mkimpl!{impl < S : Stage > SingleAttributeParser < S > for CustomMirParser { const PATH : & [rustc_span :: Symbol] = & [sym :: custom_mir] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn)]) ; const TEMPLATE : AttributeTemplate = template ! (List : & [r#"dialect = "...", phase = "...""#]) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let Some (list) = args . list () else { cx . expected_list (cx . attr_span) ; return None ; } ; let mut dialect = None ; let mut phase = None ; let mut failed = false ; for item in list . mixed () { let Some (meta_item) = item . meta_item () else { cx . expected_name_value (item . span () , None) ; failed = true ; break ; } ; if let Some (arg) = meta_item . word_is (sym :: dialect) { extract_value (cx , sym :: dialect , arg , meta_item . span () , & mut dialect , & mut failed) ; } else if let Some (arg) = meta_item . word_is (sym :: phase) { extract_value (cx , sym :: phase , arg , meta_item . span () , & mut phase , & mut failed) ; } else if let Some (word) = meta_item . path () . word () { let word = word . to_string () ; cx . unknown_key (meta_item . span () , word , & ["dialect" , "phase"]) ; failed = true ; } else { cx . expected_name_value (meta_item . span () , None) ; failed = true ; } ; } let dialect = parse_dialect (cx , dialect , & mut failed) ; let phase = parse_phase (cx , phase , & mut failed) ; if failed { return None ; } Some (AttributeKind :: CustomMir (dialect , phase , cx . attr_span)) } }}}

macro_rules! extract_value_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extract_value in module {}", module_path!());
    };
}

mkfn!{
    extract_value_introspect!();
    fn extract_value < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , key : Symbol , arg : & ArgParser < '_ > , span : Span , out_val : & mut Option < (Symbol , Span) > , failed : & mut bool ,) { if out_val . is_some () { cx . duplicate_key (span , key) ; * failed = true ; return ; } let Some (val) = arg . name_value () else { cx . expected_single_argument (arg . span () . unwrap_or (span)) ; * failed = true ; return ; } ; let Some (value_sym) = val . value_as_str () else { cx . expected_string_literal (val . value_span , Some (val . value_as_lit ())) ; * failed = true ; return ; } ; * out_val = Some ((value_sym , val . value_span)) ; }
}

macro_rules! parse_dialect_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_dialect in module {}", module_path!());
    };
}

mkfn!{
    parse_dialect_introspect!();
    fn parse_dialect < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , dialect : Option < (Symbol , Span) > , failed : & mut bool ,) -> Option < (MirDialect , Span) > { let (dialect , span) = dialect ? ; let dialect = match dialect { sym :: analysis => MirDialect :: Analysis , sym :: built => MirDialect :: Built , sym :: runtime => MirDialect :: Runtime , _ => { cx . expected_specific_argument (span , & [sym :: analysis , sym :: built , sym :: runtime]) ; * failed = true ; return None ; } } ; Some ((dialect , span)) }
}

macro_rules! parse_phase_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_phase in module {}", module_path!());
    };
}

mkfn!{
    parse_phase_introspect!();
    fn parse_phase < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , phase : Option < (Symbol , Span) > , failed : & mut bool ,) -> Option < (MirPhase , Span) > { let (phase , span) = phase ? ; let phase = match phase { sym :: initial => MirPhase :: Initial , sym :: post_cleanup => MirPhase :: PostCleanup , sym :: optimized => MirPhase :: Optimized , _ => { cx . expected_specific_argument (span , & [sym :: initial , sym :: post_cleanup , sym :: optimized]) ; * failed = true ; return None ; } } ; Some ((phase , span)) }
}