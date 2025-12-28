macro_rules! db {
    () => {
        # [cfg (feature = "db")] pub mod db ;
    };
}

db!();