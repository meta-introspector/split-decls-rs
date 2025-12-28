macro_rules! deps {
    () => {
        TypeVariableValue!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < 'tcx > ut :: UnifyValue for TypeVariableValue < 'tcx > { type Error = ut :: NoError ; fn unify_values (value1 : & Self , value2 : & Self) -> Result < Self , ut :: NoError > { match (value1 , value2) { (& TypeVariableValue :: Known { .. } , & TypeVariableValue :: Known { .. }) => { bug ! ("equating two type variables, both of which have known types") } (& TypeVariableValue :: Known { .. } , & TypeVariableValue :: Unknown { .. }) => Ok (* value1) , (& TypeVariableValue :: Unknown { .. } , & TypeVariableValue :: Known { .. }) => Ok (* value2) , (& TypeVariableValue :: Unknown { universe : universe1 } , & TypeVariableValue :: Unknown { universe : universe2 } ,) => { let universe = cmp :: min (universe1 , universe2) ; Ok (TypeVariableValue :: Unknown { universe }) } } } }
    };
}

impl_231!();