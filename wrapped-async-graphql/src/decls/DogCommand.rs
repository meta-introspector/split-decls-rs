macro_rules! DogCommand {
    () => {
        # [derive (Enum , Eq , PartialEq , Copy , Clone)] # [graphql (internal)] enum DogCommand { Sit , Heel , Down , }
    };
}

DogCommand!();