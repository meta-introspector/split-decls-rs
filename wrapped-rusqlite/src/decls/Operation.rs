macro_rules! deps {
    () => {
        Action!();
    };
}

macro_rules! Operation {
    () => {
        deps!();
        # [doc = " Operation"] pub struct Operation < 'item > { table_name : & 'item str , number_of_columns : i32 , code : Action , indirect : bool , }
    };
}

Operation!()