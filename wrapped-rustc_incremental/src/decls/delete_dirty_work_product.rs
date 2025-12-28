macro_rules! deps {
    () => {
        SerializedWorkProduct!();
    };
}

macro_rules! delete_dirty_work_product {
    () => {
        deps!();
        fn delete_dirty_work_product (sess : & Session , swp : SerializedWorkProduct) { debug ! ("delete_dirty_work_product({:?})" , swp) ; work_product :: delete_workproduct_files (sess , & swp . work_product) ; }
    };
}

delete_dirty_work_product!()