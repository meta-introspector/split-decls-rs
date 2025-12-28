macro_rules! deps {
    () => {
        ContainerHandle!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl ContainerHandle { # [doc = " Executes a program inside a running container."] pub fn exec (& self , args : & [& str]) -> std :: process :: Output { ProcessBuilder :: new ("docker") . args (& ["container" , "exec" , & self . name]) . args (args) . exec_with_output () . unwrap () } # [doc = " Returns the contents of a file inside the container."] pub fn read_file (& self , path : & str) -> String { let output = ProcessBuilder :: new ("docker") . args (& ["cp" , & format ! ("{}:{}" , self . name , path) , "-"]) . exec_with_output () . unwrap () ; let mut ar = tar :: Archive :: new (output . stdout . as_slice ()) ; let mut entry = ar . entries () . unwrap () . next () . unwrap () . unwrap () ; let mut contents = String :: new () ; entry . read_to_string (& mut contents) . unwrap () ; contents } }
    };
}

impl_35!()