macro_rules! deps {
    () => {
        SerializedWorkProduct!();
    };
}

macro_rules! encode_work_product_index {
    () => {
        deps!();
        fn encode_work_product_index (work_products : & FxIndexMap < WorkProductId , WorkProduct > , encoder : & mut FileEncoder ,) { let serialized_products : Vec < _ > = work_products . iter () . map (| (id , work_product) | SerializedWorkProduct { id : * id , work_product : work_product . clone () , }) . collect () ; serialized_products . encode (encoder) }
    };
}

encode_work_product_index!()