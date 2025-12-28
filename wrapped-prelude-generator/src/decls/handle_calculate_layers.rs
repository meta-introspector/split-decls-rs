macro_rules! deps {
    () => {
        Args!();
        TypeInfo!();
    };
}

macro_rules! handle_calculate_layers {
    () => {
        deps!();
        pub async fn handle_calculate_layers (project_root : & PathBuf , args : & crate :: Args ,) -> anyhow :: Result < () > { println ! ("Calculating type layers...") ; let type_map = type_extractor :: extract_bag_of_types (project_root , & args . filter_names) . await ? ; println ! ("\n--- Type Layer Analysis ---") ; println ! ("{:<30} {:<10} {:<10}" , "Type" , "Count" , "Layer") ; println ! ("---------------------------------------------------") ; let mut sorted_types : Vec < (& String , & type_extractor :: TypeInfo) > = type_map . iter () . collect () ; sorted_types . sort_by (| a , b | b . 1 . count . cmp (& a . 1 . count)) ; for (type_name , info) in sorted_types . iter () { println ! ("{:<30} {:<10} {:<10?}" , type_name , info . count , info . layer) ; } println ! ("---------------------------------------------------") ; Ok (()) }
    };
}

handle_calculate_layers!()