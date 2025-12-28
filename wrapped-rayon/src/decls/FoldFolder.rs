macro_rules! FoldFolder {
    () => {
        struct FoldFolder < 'r , C , ID , F > { base : C , fold_op : & 'r F , item : ID , }
    };
}

FoldFolder!()