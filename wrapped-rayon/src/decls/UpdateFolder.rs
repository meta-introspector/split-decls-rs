macro_rules! UpdateFolder {
    () => {
        struct UpdateFolder < 'f , C , F > { base : C , update_op : & 'f F , }
    };
}

UpdateFolder!();