macro_rules! deps {
    () => {
        Ok!();
        CreateIncrCompDir!();
    };
}

macro_rules! create_dir {
    () => {
        deps!();
        fn create_dir (sess : & Session , path : & Path , dir_tag : & str) { match std_fs :: create_dir_all (path) { Ok (()) => { debug ! ("{} directory created successfully" , dir_tag) ; } Err (err) => sess . dcx () . emit_fatal (errors :: CreateIncrCompDir { tag : dir_tag , path , err }) , } }
    };
}

create_dir!();