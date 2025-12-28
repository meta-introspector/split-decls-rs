macro_rules! deps {
    () => {
        ProcMacroExpansionError!();
    };
}

macro_rules! ProcMacroExpander {
    () => {
        deps!();
        # [doc = " A proc-macro expander implementation."] pub trait ProcMacroExpander : fmt :: Debug + Send + Sync + RefUnwindSafe + Any { # [doc = " Run the expander with the given input subtree, optional attribute input subtree (for"] # [doc = " [`ProcMacroKind::Attr`]), environment variables, and span information."] fn expand (& self , subtree : & tt :: TopSubtree , attrs : Option < & tt :: TopSubtree > , env : & Env , def_site : Span , call_site : Span , mixed_site : Span , current_dir : String ,) -> Result < tt :: TopSubtree , ProcMacroExpansionError > ; fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool ; }
    };
}

ProcMacroExpander!();