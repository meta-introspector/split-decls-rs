macro_rules! IsNull {
    () => {
        # [doc (hidden)] pub trait IsNull { fn is_ptr_null (& self) -> bool ; }
    };
}

IsNull!()