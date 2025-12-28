macro_rules! deps {
    () => {
        ProjectBuilder!();
    };
}

macro_rules! project {
    () => {
        deps!();
        # [doc = " Generates a project layout, see [`ProjectBuilder`]"] pub fn project () -> ProjectBuilder { ProjectBuilder :: new (paths :: root () . join ("foo")) }
    };
}

project!()