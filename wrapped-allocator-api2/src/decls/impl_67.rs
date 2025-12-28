macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < F : ? Sized + Future + Unpin , A : Allocator > Future for Box < F , A > where A : 'static , { type Output = F :: Output ; # [inline (always)] fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { F :: poll (Pin :: new (& mut * self) , cx) } }
    };
}

impl_67!()