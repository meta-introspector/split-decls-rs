macro_rules! deps {
    () => {
        ParseError!();
        FloatLit!();
    };
}

macro_rules! manual_without_suffix {
    () => {
        deps!();
        # [test] fn manual_without_suffix () -> Result < () , ParseError > { let f = FloatLit :: parse ("3.14") ? ; assert_eq ! (f . number_part () , "3.14") ; assert_eq ! (f . integer_part () , "3") ; assert_eq ! (f . fractional_part () , Some ("14")) ; assert_eq ! (f . exponent_part () , "") ; assert_eq ! (f . suffix () , "") ; let f = FloatLit :: parse ("9.") ? ; assert_eq ! (f . number_part () , "9.") ; assert_eq ! (f . integer_part () , "9") ; assert_eq ! (f . fractional_part () , Some ("")) ; assert_eq ! (f . exponent_part () , "") ; assert_eq ! (f . suffix () , "") ; let f = FloatLit :: parse ("8e1") ? ; assert_eq ! (f . number_part () , "8e1") ; assert_eq ! (f . integer_part () , "8") ; assert_eq ! (f . fractional_part () , None) ; assert_eq ! (f . exponent_part () , "e1") ; assert_eq ! (f . suffix () , "") ; let f = FloatLit :: parse ("8E3") ? ; assert_eq ! (f . number_part () , "8E3") ; assert_eq ! (f . integer_part () , "8") ; assert_eq ! (f . fractional_part () , None) ; assert_eq ! (f . exponent_part () , "E3") ; assert_eq ! (f . suffix () , "") ; let f = FloatLit :: parse ("8_7_6.1_23e15") ? ; assert_eq ! (f . number_part () , "8_7_6.1_23e15") ; assert_eq ! (f . integer_part () , "8_7_6") ; assert_eq ! (f . fractional_part () , Some ("1_23")) ; assert_eq ! (f . exponent_part () , "e15") ; assert_eq ! (f . suffix () , "") ; let f = FloatLit :: parse ("8.2e-_04_9") ? ; assert_eq ! (f . number_part () , "8.2e-_04_9") ; assert_eq ! (f . integer_part () , "8") ; assert_eq ! (f . fractional_part () , Some ("2")) ; assert_eq ! (f . exponent_part () , "e-_04_9") ; assert_eq ! (f . suffix () , "") ; Ok (()) }
    };
}

manual_without_suffix!();