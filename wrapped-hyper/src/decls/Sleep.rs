macro_rules! deps {
    () => {
        Timer!();
    };
}

macro_rules! Sleep {
    () => {
        deps!();
        # [doc = " A future returned by a `Timer`."] pub trait Sleep : Send + Sync + Future < Output = () > { # [doc (hidden)] # [doc = " This method is private and can not be implemented by downstream crate"] fn __type_id (& self , _ : private :: Sealed) -> TypeId where Self : 'static , { TypeId :: of :: < Self > () } }
    };
}

Sleep!()