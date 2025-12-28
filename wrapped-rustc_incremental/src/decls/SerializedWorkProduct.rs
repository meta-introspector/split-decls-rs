macro_rules! SerializedWorkProduct {
    () => {
        # [derive (Debug , Encodable , Decodable)] pub (crate) struct SerializedWorkProduct { # [doc = " node that produced the work-product"] pub id : WorkProductId , # [doc = " work-product data itself"] pub work_product : WorkProduct , }
    };
}

SerializedWorkProduct!()