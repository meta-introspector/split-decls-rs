macro_rules! deps {
    () => {
        LeafName!();
        TemplateArg!();
        ArgScope!();
        Result!();
        TemplateArgs!();
        ClosureTypeName!();
        Error!();
        Type!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl < 'subs > ArgScope < 'subs , 'subs > for ClosureTypeName { fn leaf_name (& 'subs self) -> Result < LeafName < 'subs > > { Ok (LeafName :: Closure (self)) } fn get_template_arg (& 'subs self , _ : usize ,) -> Result < (& 'subs TemplateArg , & 'subs TemplateArgs) > { Err (error :: Error :: BadTemplateArgReference) } fn get_function_arg (& 'subs self , _ : usize) -> Result < & 'subs Type > { Err (error :: Error :: BadFunctionArgReference) } }
    };
}

impl_281!()