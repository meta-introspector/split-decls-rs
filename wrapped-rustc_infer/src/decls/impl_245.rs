macro_rules! deps {
    () => {
        ConstVariableValue!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < 'tcx > UnifyValue for ConstVariableValue < 'tcx > { type Error = NoError ; fn unify_values (& value1 : & Self , & value2 : & Self) -> Result < Self , Self :: Error > { match (value1 , value2) { (ConstVariableValue :: Known { .. } , ConstVariableValue :: Known { .. }) => { bug ! ("equating two const variables, both of which have known values") } (ConstVariableValue :: Known { .. } , ConstVariableValue :: Unknown { .. }) => Ok (value1) , (ConstVariableValue :: Unknown { .. } , ConstVariableValue :: Known { .. }) => Ok (value2) , (ConstVariableValue :: Unknown { origin , universe : universe1 } , ConstVariableValue :: Unknown { origin : _ , universe : universe2 } ,) => { let universe = cmp :: min (universe1 , universe2) ; Ok (ConstVariableValue :: Unknown { origin , universe }) } } } }
    };
}

impl_245!()