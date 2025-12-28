macro_rules! deps {
    () => {
        LazyAttrTokenStreamInner!();
        AttrTokenStream!();
    };
}

macro_rules! LazyAttrTokenStream {
    () => {
        deps!();
        # [doc = " A lazy version of [`AttrTokenStream`], which defers creation of an actual"] # [doc = " `AttrTokenStream` until it is needed."] # [derive (Clone)] pub struct LazyAttrTokenStream (Arc < LazyAttrTokenStreamInner >) ;
    };
}

LazyAttrTokenStream!()