macro_rules! FurColor {
    () => {
        # [derive (Enum , Copy , Clone , Eq , PartialEq)] # [graphql (internal)] enum FurColor { Brown , Black , Tan , Spotted , }
    };
}

FurColor!()