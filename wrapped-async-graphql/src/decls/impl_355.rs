macro_rules! deps {
    () => {
        SelectionField!();
        ServerResult!();
        SelectionFieldsIter!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl < 'a > SelectionField < 'a > { # [doc = " Get the name of this field."] # [inline] pub fn name (& self) -> & 'a str { self . field . name . node . as_str () } # [doc = " Get the alias of this field."] # [inline] pub fn alias (& self) -> Option < & 'a str > { self . field . alias . as_ref () . map (| alias | alias . node . as_str ()) } # [doc = " Get the directives of this field."] pub fn directives (& self) -> ServerResult < Vec < ConstDirective > > { let mut directives = Vec :: with_capacity (self . field . directives . len ()) ; for directive in & self . field . directives { let directive = & directive . node ; let mut arguments = Vec :: with_capacity (directive . arguments . len ()) ; for (name , value) in & directive . arguments { let pos = name . pos ; arguments . push ((name . clone () , value . position_node (value . node . clone () . into_const_with (| name | self . context . var_value (& name , pos)) ? ,) ,)) ; } directives . push (ConstDirective { name : directive . name . clone () , arguments , }) ; } Ok (directives) } # [doc = " Get the arguments of this field."] pub fn arguments (& self) -> ServerResult < Vec < (Name , Value) > > { let mut arguments = Vec :: with_capacity (self . field . arguments . len ()) ; for (name , value) in & self . field . arguments { let pos = name . pos ; arguments . push ((name . node . clone () , value . clone () . node . into_const_with (| name | self . context . var_value (& name , pos)) ? ,)) ; } Ok (arguments) } # [doc = " Get all subfields of the current selection set."] pub fn selection_set (& self) -> impl Iterator < Item = SelectionField < 'a > > { SelectionFieldsIter { fragments : self . fragments , iter : vec ! [self . field . selection_set . node . items . iter ()] , context : self . context , } } }
    };
}

impl_355!()