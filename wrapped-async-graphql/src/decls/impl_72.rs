macro_rules! deps {
    () => {
        SelectionField!();
        Lookahead!();
        Error!();
        Result!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        # [doc = " Convert a slice of `SelectionField`s to a `Lookahead`."] # [doc = " Assumes all `SelectionField`s are from the same query and thus have the same"] # [doc = " fragments."] # [doc = ""] # [doc = " Fails if either no `SelectionField`s were provided."] impl < 'a > TryFrom < & [SelectionField < 'a >] > for Lookahead < 'a > { type Error = () ; fn try_from (selection_fields : & [SelectionField < 'a >]) -> Result < Self , Self :: Error > { if selection_fields . is_empty () { Err (()) } else { Ok (Lookahead { fragments : selection_fields [0] . fragments , fields : selection_fields . iter () . map (| selection_field | selection_field . field) . collect () , context : selection_fields [0] . context , }) } } }
    };
}

impl_72!();