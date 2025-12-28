macro_rules! InterfaceMethodArg {
    () => {
        # [doc = " An argument to an interface method"] struct InterfaceMethodArg { # [doc = " The type of the argument"] pub ty : Box < syn :: Type > , # [doc = " The name of the argument"] pub pat : Box < syn :: Pat > , }
    };
}

InterfaceMethodArg!();