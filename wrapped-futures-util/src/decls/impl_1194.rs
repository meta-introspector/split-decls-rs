macro_rules! deps {
    () => {
        Seek!();
    };
}

macro_rules! impl_1194 {
    () => {
        deps!();
        impl < S : AsyncSeek + ? Sized + Unpin > Future for Seek < '_ , S > { type Output = io :: Result < u64 > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; Pin :: new (& mut this . seek) . poll_seek (cx , this . pos) } }
    };
}

impl_1194!()