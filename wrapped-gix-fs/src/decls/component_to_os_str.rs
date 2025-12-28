macro_rules! component_to_os_str {
    () => {
        fn component_to_os_str < 'a > (component : Component < 'a > , path_with_component : & Path ,) -> Result < & 'a OsStr , to_normal_path_components :: Error > { match component { Component :: Normal (os_str) => Ok (os_str) , _ => Err (to_normal_path_components :: Error :: NotANormalComponent (path_with_component . to_owned () ,)) , } }
    };
}

component_to_os_str!()