macro_rules! loader {
    () => {
        # [cfg (feature = "loader")] mod loader ;
    };
}

loader!()