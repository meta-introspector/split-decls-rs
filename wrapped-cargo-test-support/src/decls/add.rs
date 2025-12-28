macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! add {
    () => {
        deps!();
        # [doc = " *(`git2`)* Add all files in the working directory to the git index"] pub fn add (repo : & git2 :: Repository) { let mut index = t ! (repo . index ()) ; t ! (index . add_all (["*"] . iter () , git2 :: IndexAddOption :: DEFAULT , None)) ; t ! (index . write ()) ; }
    };
}

add!();