macro_rules! deps {
    () => {
        Dependency!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl Dependency { pub fn new (name : & str , vers : & str) -> Dependency { Dependency { name : name . to_string () , vers : vers . to_string () , kind : "normal" . to_string () , artifact : None , bindep_target : None , lib : false , target : None , features : Vec :: new () , package : None , optional : false , registry : None , default_features : true , public : false , } } # [doc = " Changes this to `[build-dependencies]`."] pub fn build (& mut self) -> & mut Self { self . kind = "build" . to_string () ; self } # [doc = " Changes this to `[dev-dependencies]`."] pub fn dev (& mut self) -> & mut Self { self . kind = "dev" . to_string () ; self } # [doc = " Changes this to `[target.$target.dependencies]`."] pub fn target (& mut self , target : & str) -> & mut Self { self . target = Some (target . to_string ()) ; self } # [doc = " Change the artifact to be of the given kind, like \"bin\", or \"staticlib\","] # [doc = " along with a specific target triple if provided."] pub fn artifact (& mut self , kind : & str , target : Option < String >) -> & mut Self { self . artifact = Some (kind . to_string ()) ; self . bindep_target = target ; self } # [doc = " Adds `registry = $registry` to this dependency."] pub fn registry (& mut self , registry : & str) -> & mut Self { self . registry = Some (registry . to_string ()) ; self } # [doc = " Adds `features = [ ... ]` to this dependency."] pub fn enable_features (& mut self , features : & [& str]) -> & mut Self { self . features . extend (features . iter () . map (| s | s . to_string ())) ; self } # [doc = " Adds `package = ...` to this dependency."] pub fn package (& mut self , pkg : & str) -> & mut Self { self . package = Some (pkg . to_string ()) ; self } # [doc = " Changes this to an optional dependency."] pub fn optional (& mut self , optional : bool) -> & mut Self { self . optional = optional ; self } # [doc = " Changes this to an public dependency."] pub fn public (& mut self , public : bool) -> & mut Self { self . public = public ; self } # [doc = " Adds `default-features = false` if the argument is `false`."] pub fn default_features (& mut self , default_features : bool) -> & mut Self { self . default_features = default_features ; self } }
    };
}

impl_140!()