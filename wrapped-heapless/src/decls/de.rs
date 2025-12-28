macro_rules! de {
    () => {
        # [cfg (feature = "serde")] mod de ;
    };
}

de!()