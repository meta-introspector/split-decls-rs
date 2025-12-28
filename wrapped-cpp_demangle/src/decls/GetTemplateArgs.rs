macro_rules! deps {
    () => {
        TemplateArgs!();
        SubstitutionTable!();
    };
}

macro_rules! GetTemplateArgs {
    () => {
        deps!();
        # [doc = " Determine whether this AST node is an instantiated[*] template function, and"] # [doc = " get its concrete template arguments."] # [doc = ""] # [doc = " [*] Note that we will never see an abstract, un-instantiated template"] # [doc = " function, since they don't end up in object files and don't get mangled"] # [doc = " names."] trait GetTemplateArgs { # [doc = " Returns `Some` if this is a template function, `None` otherwise."] fn get_template_args < 'a > (& 'a self , subs : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > ; }
    };
}

GetTemplateArgs!()