macro_rules! deps {
    () => {
        TemplateArgs!();
        DemangleWrite!();
        DemangleContext!();
        Result!();
        DemangleNodeType!();
        Demangle!();
        ArgScopeStack!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for TemplateArgs where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , mut scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; inner_barrier ! (ctx) ; if ctx . last_char_written == Some ('<') { write ! (ctx , " ") ? ; } write ! (ctx , "<") ? ; ctx . push_demangle_node (DemangleNodeType :: TemplateArgs) ; let mut need_comma = false ; for arg_index in 0 .. self . 0 . len () { if need_comma { write ! (ctx , ", ") ? ; } if let Some (ref mut scope) = scope { scope . in_arg = Some ((arg_index , self)) ; } self . 0 [arg_index] . demangle (ctx , scope) ? ; need_comma = true ; } if ctx . last_char_written == Some ('>') { write ! (ctx , " ") ? ; } ctx . pop_demangle_node () ; write ! (ctx , ">") ? ; Ok (()) } }
    };
}

impl_234!();