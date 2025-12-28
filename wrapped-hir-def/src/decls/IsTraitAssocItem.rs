macro_rules! IsTraitAssocItem {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , Ord , PartialOrd)] enum IsTraitAssocItem { Yes , No , }
    };
}

IsTraitAssocItem!();