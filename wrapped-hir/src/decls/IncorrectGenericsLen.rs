macro_rules! deps {
    () => {
        GenericDef!();
    };
}

macro_rules! IncorrectGenericsLen {
    () => {
        deps!();
        # [derive (Debug)] pub struct IncorrectGenericsLen { # [doc = " Points at the name if there are no generics."] pub generics_or_segment : InFile < AstPtr < Either < ast :: GenericArgList , ast :: NameRef > > > , pub kind : IncorrectGenericsLenKind , pub provided : u32 , pub expected : u32 , pub def : GenericDef , }
    };
}

IncorrectGenericsLen!();