macro_rules! Str {
    () => {
        # [doc = " A UTF-8-encoded fixed string"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **NOTE:** To support dynamic values (i.e. `String`), enable the `string`"] # [doc = " feature"] # [doc = ""] # [doc = " </div>"] # [derive (Default , Clone , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct Str { name : Inner , }
    };
}

Str!();