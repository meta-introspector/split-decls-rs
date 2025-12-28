macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! tag {
    () => {
        deps!();
        # [doc = " *(`git2`)* Create a new tag in the git repository"] pub fn tag (repo : & git2 :: Repository , name : & str) { let head = repo . head () . unwrap () . target () . unwrap () ; t ! (repo . tag (name , & t ! (repo . find_object (head , None)) , & t ! (repo . signature ()) , "make a new tag" , false)) ; }
    };
}

tag!()