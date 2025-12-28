macro_rules! deps {
    () => {
        Registry!();
        MetaDirective!();
        Directive!();
    };
}

macro_rules! __Directive {
    () => {
        deps!();
        pub struct __Directive < 'a > { pub registry : & 'a registry :: Registry , pub visible_types : & 'a HashSet < & 'a str > , pub directive : & 'a registry :: MetaDirective , }
    };
}

__Directive!();