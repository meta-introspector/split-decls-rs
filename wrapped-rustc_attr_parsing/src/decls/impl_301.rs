macro_rules! deps {
    () => {
        ShouldEmit!();
        PathParser!();
        MetaItemParser!();
        ArgParser!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl < 'a > MetaItemParser < 'a > { # [doc = " Create a new parser from a [`NormalAttr`], which is stored inside of any"] # [doc = " [`ast::Attribute`](rustc_ast::Attribute)"] pub fn from_attr < 'sess > (attr : & 'a NormalAttr , parts : & [Symbol] , psess : & 'sess ParseSess , should_emit : ShouldEmit ,) -> Option < Self > { Some (Self { path : PathParser (Cow :: Borrowed (& attr . item . path)) , args : ArgParser :: from_attr_args (& attr . item . args , parts , psess , should_emit) ? , }) } }
    };
}

impl_301!()