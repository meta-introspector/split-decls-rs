macro_rules! emit_delayed_lint {
    () => {
        fn emit_delayed_lint (lint : & DelayedLint , tcx : TyCtxt < '_ >) { match lint { DelayedLint :: AttributeParsing (attribute_lint) => { rustc_attr_parsing :: emit_attribute_lint (attribute_lint , tcx) } } }
    };
}

emit_delayed_lint!();