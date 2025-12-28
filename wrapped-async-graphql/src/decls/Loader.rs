macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! Loader {
    () => {
        deps!();
        # [doc = " Trait for batch loading."] # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] pub trait Loader < K : Send + Sync + Hash + Eq + Clone + 'static > : Send + Sync + 'static { # [doc = " type of value."] type Value : Send + Sync + Clone + 'static ; # [doc = " Type of error."] type Error : Send + Clone + 'static ; # [doc = " Load the data set specified by the `keys`."] # [cfg (feature = "boxed-trait")] async fn load (& self , keys : & [K]) -> Result < HashMap < K , Self :: Value > , Self :: Error > ; # [doc = " Load the data set specified by the `keys`."] # [cfg (not (feature = "boxed-trait"))] fn load (& self , keys : & [K] ,) -> impl Future < Output = Result < HashMap < K , Self :: Value > , Self :: Error > > + Send ; }
    };
}

Loader!()