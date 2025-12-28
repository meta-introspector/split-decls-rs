macro_rules! macro_5 {
    () => {
        mkbuildrs ! { module_name : "grast_database" ; dependencies : ["std::collections::HashMap" , "std::fs" , "std::io" , "std::path::Path"] ; description : "Database operations for grast triples with indexing and file I/O" ; }
    };
}

macro_5!()