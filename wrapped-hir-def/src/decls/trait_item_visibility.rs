macro_rules! deps {
    () => {
        DefDatabase!();
        Visibility!();
        ItemContainerId!();
    };
}

macro_rules! trait_item_visibility {
    () => {
        deps!();
        fn trait_item_visibility (db : & dyn DefDatabase , container : ItemContainerId) -> Option < Visibility > { match container { ItemContainerId :: TraitId (trait_) => Some (trait_visibility (db , trait_)) , _ => None , } }
    };
}

trait_item_visibility!()