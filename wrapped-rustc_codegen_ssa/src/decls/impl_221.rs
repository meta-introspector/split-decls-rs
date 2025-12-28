macro_rules! deps {
    () => {
        WorkItem!();
        WriteBackendMethods!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl < B : WriteBackendMethods > WorkItem < B > { # [doc = " Generate a short description of this work item suitable for use as a thread name."] fn short_description (& self) -> String { # [cfg (not (windows))] fn desc (short : & str , _long : & str , name : & str) -> String { assert_eq ! (short . len () , 3) ; let name = if let Some (index) = name . find ("-cgu.") { & name [index + 1 ..] } else { name } ; format ! ("{short} {name}") } # [cfg (windows)] fn desc (_short : & str , long : & str , name : & str) -> String { format ! ("{long} {name}") } match self { WorkItem :: Optimize (m) => desc ("opt" , "optimize module" , & m . name) , WorkItem :: CopyPostLtoArtifacts (m) => desc ("cpy" , "copy LTO artifacts for" , & m . name) , WorkItem :: FatLto { .. } => desc ("lto" , "fat LTO module" , "everything") , WorkItem :: ThinLto (m) => desc ("lto" , "thin-LTO module" , m . name ()) , } } }
    };
}

impl_221!()