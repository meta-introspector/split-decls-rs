macro_rules! CompassPt {
    () => {
        # [doc = " An enum that corresponds to the `compass_pt` non-terminal of the grammar."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Hash , Copy , Clone)] pub enum CompassPt { # [doc = " A North orientation"] N , # [doc = " A North-East orientation"] NE , # [doc = " An East orientation"] E , # [doc = " A South-East orientation"] SE , # [doc = " A South orientation"] S , # [doc = " A South-West orientation"] SW , # [doc = " A West orientation"] W , # [doc = " A North-West orientation"] NW , # [doc = " A Central orientation"] C , # [doc = " An unspecified orientation"] Underscore , }
    };
}

CompassPt!()