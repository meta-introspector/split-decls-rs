macro_rules! deps {
    () => {
        GenericDef!();
    };
}

macro_rules! MissingLifetime {
    () => {
        deps!();
        # [derive (Debug)] pub struct MissingLifetime { # [doc = " Points at the name if there are no generics."] pub generics_or_segment : InFile < AstPtr < Either < ast :: GenericArgList , ast :: NameRef > > > , pub expected : u32 , pub def : GenericDef , }
    };
}

MissingLifetime!();