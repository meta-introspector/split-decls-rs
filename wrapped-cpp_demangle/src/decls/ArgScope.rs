macro_rules! deps {
    () => {
        TemplateArg!();
        Result!();
        TemplateArgs!();
        LeafName!();
        Type!();
    };
}

macro_rules! ArgScope {
    () => {
        deps!();
        # [doc = " When formatting a mangled symbol's parsed AST as a demangled symbol, we need"] # [doc = " to resolve indirect references to template and function arguments with"] # [doc = " direct `TemplateArg` and `Type` references respectively."] # [doc = ""] # [doc = " Note that which set of arguments are implicitly referenced change as we"] # [doc = " enter and leave different functions' scope. One might usually use de Brujin"] # [doc = " indices to keep arguments within scopes separated from each other, but the"] # [doc = " Itanium C++ ABI does not allow us the luxury. AFAIK, when the ABI was first"] # [doc = " drafted, C++ did not have lambdas, and the issue did not come up at all"] # [doc = " since a function simply couldn't refer to the types of closed over"] # [doc = " variables."] # [doc = ""] # [doc = " This trait is implemented by anything that can potentially resolve arguments"] # [doc = " for us."] trait ArgScope < 'me , 'ctx > : fmt :: Debug { # [doc = " Get the current scope's leaf name."] fn leaf_name (& 'me self) -> Result < LeafName < 'ctx > > ; # [doc = " Get the current scope's `index`th template argument."] fn get_template_arg (& 'me self , index : usize) -> Result < (& 'ctx TemplateArg , & 'ctx TemplateArgs) > ; # [allow (unused)] # [doc = " Get the current scope's `index`th function argument's type."] fn get_function_arg (& 'me self , index : usize) -> Result < & 'ctx Type > ; }
    };
}

ArgScope!()