macro_rules! deps {
    () => {
        ExpandResult!();
        MacroArgResult!();
        HirFileId!();
        MacroCallId!();
        ExpansionSpanMap!();
        DeclarativeMacroExpander!();
        SpanMap!();
        MacroCallLoc!();
        AstId!();
        ProcMacros!();
        MacroDefId!();
        CrateProcMacros!();
        TokenExpander!();
        MacroCallKind!();
    };
}

macro_rules! ExpandDatabase {
    () => {
        deps!();
        # [query_group :: query_group] pub trait ExpandDatabase : RootQueryDb { # [doc = " The proc macros. Do not use this! Use `proc_macros_for_crate()` instead."] # [salsa :: input] fn proc_macros (& self) -> Arc < ProcMacros > ; # [doc = " Incrementality query to prevent queries from directly depending on `ExpandDatabase::proc_macros`."] # [salsa :: invoke (crate :: proc_macro :: proc_macros_for_crate)] fn proc_macros_for_crate (& self , krate : Crate) -> Option < Arc < CrateProcMacros > > ; # [salsa :: invoke (ast_id_map)] # [salsa :: lru (1024)] fn ast_id_map (& self , file_id : HirFileId) -> Arc < AstIdMap > ; # [salsa :: transparent] fn parse_or_expand (& self , file_id : HirFileId) -> SyntaxNode ; # [doc = " Implementation for the macro case."] # [salsa :: lru (512)] fn parse_macro_expansion (& self , macro_file : MacroCallId ,) -> ExpandResult < (Parse < SyntaxNode > , Arc < ExpansionSpanMap >) > ; # [salsa :: transparent] # [salsa :: invoke (SpanMap :: new)] fn span_map (& self , file_id : HirFileId) -> SpanMap ; # [salsa :: transparent] # [salsa :: invoke (crate :: span_map :: expansion_span_map)] fn expansion_span_map (& self , file_id : MacroCallId) -> Arc < ExpansionSpanMap > ; # [salsa :: invoke (crate :: span_map :: real_span_map)] fn real_span_map (& self , file_id : EditionedFileId) -> Arc < RealSpanMap > ; # [doc = " Macro ids. That's probably the tricksiest bit in rust-analyzer, and the"] # [doc = " reason why we use salsa at all."] # [doc = ""] # [doc = " We encode macro definitions into ids of macro calls, this what allows us"] # [doc = " to be incremental."] # [salsa :: transparent] fn intern_macro_call (& self , macro_call : MacroCallLoc) -> MacroCallId ; # [salsa :: transparent] fn lookup_intern_macro_call (& self , macro_call : MacroCallId) -> MacroCallLoc ; # [doc = " Lowers syntactic macro call to a token tree representation. That's a firewall"] # [doc = " query, only typing in the macro call itself changes the returned"] # [doc = " subtree."] # [deprecated = "calling this is incorrect, call `macro_arg_considering_derives` instead"] # [salsa :: invoke (macro_arg)] fn macro_arg (& self , id : MacroCallId) -> MacroArgResult ; # [salsa :: transparent] fn macro_arg_considering_derives (& self , id : MacroCallId , kind : & MacroCallKind ,) -> MacroArgResult ; # [doc = " Fetches the expander for this macro."] # [salsa :: transparent] # [salsa :: invoke (TokenExpander :: macro_expander)] fn macro_expander (& self , id : MacroDefId) -> TokenExpander ; # [doc = " Fetches (and compiles) the expander of this decl macro."] # [salsa :: invoke (DeclarativeMacroExpander :: expander)] fn decl_macro_expander (& self , def_crate : Crate , id : AstId < ast :: Macro > ,) -> Arc < DeclarativeMacroExpander > ; # [doc = " Special case of the previous query for procedural macros. We can't LRU"] # [doc = " proc macros, since they are not deterministic in general, and"] # [doc = " non-determinism breaks salsa in a very, very, very bad way."] # [doc = " @edwin0cheng heroically debugged this once! See #4315 for details"] # [salsa :: invoke (expand_proc_macro)] fn expand_proc_macro (& self , call : MacroCallId) -> ExpandResult < Arc < tt :: TopSubtree > > ; # [doc = " Retrieves the span to be used for a proc-macro expansions spans."] # [doc = " This is a firewall query as it requires parsing the file, which we don't want proc-macros to"] # [doc = " directly depend on as that would cause to frequent invalidations, mainly because of the"] # [doc = " parse queries being LRU cached. If they weren't the invalidations would only happen if the"] # [doc = " user wrote in the file that defines the proc-macro."] # [salsa :: invoke_interned (proc_macro_span)] fn proc_macro_span (& self , fun : AstId < ast :: Fn >) -> Span ; # [doc = " Firewall query that returns the errors from the `parse_macro_expansion` query."] # [salsa :: invoke (parse_macro_expansion_error)] fn parse_macro_expansion_error (& self , macro_call : MacroCallId ,) -> Option < Arc < ExpandResult < Arc < [SyntaxError] > > > > ; # [salsa :: transparent] fn syntax_context (& self , file : HirFileId , edition : Edition) -> SyntaxContext ; }
    };
}

ExpandDatabase!();