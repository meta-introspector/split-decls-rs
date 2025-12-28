macro_rules! derive {
    () => {
        # [cfg (feature = "derive")] mod derive ;
    };
}

derive!();