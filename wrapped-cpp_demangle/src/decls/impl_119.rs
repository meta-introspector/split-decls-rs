macro_rules! deps {
    () => {
        LeafName!();
        SourceName!();
        TemplateArg!();
        Error!();
        ArgScope!();
        Result!();
        Type!();
        TemplateArgs!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'subs > ArgScope < 'subs , 'subs > for SourceName { fn leaf_name (& 'subs self) -> Result < LeafName < 'subs > > { Ok (LeafName :: SourceName (self)) } fn get_template_arg (& 'subs self , _ : usize ,) -> Result < (& 'subs TemplateArg , & 'subs TemplateArgs) > { Err (error :: Error :: BadTemplateArgReference) } fn get_function_arg (& 'subs self , _ : usize) -> Result < & 'subs Type > { Err (error :: Error :: BadFunctionArgReference) } }
    };
}

impl_119!();