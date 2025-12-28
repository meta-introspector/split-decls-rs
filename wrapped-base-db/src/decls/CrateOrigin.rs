macro_rules! deps {
    () => {
        LangCrateOrigin!();
    };
}

macro_rules! CrateOrigin {
    () => {
        deps!();
        # [doc = " Origin of the crates."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum CrateOrigin { # [doc = " Crates that are from the rustc workspace."] Rustc { name : Symbol } , # [doc = " Crates that are workspace members."] Local { repo : Option < String > , name : Option < Symbol > } , # [doc = " Crates that are non member libraries."] Library { repo : Option < String > , name : Symbol } , # [doc = " Crates that are provided by the language, like std, core, proc-macro, ..."] Lang (LangCrateOrigin) , }
    };
}

CrateOrigin!()