macro_rules! UnimplementedBuiltinMacro {
    () => {
        # [derive (Debug)] pub struct UnimplementedBuiltinMacro { pub node : InFile < SyntaxNodePtr > , }
    };
}

UnimplementedBuiltinMacro!()