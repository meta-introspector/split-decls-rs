macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl PartialOrd for BytesMut { fn partial_cmp (& self , other : & BytesMut) -> Option < cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_194!();