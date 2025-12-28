macro_rules! deps {
    () => {
        Pet!();
        Dog!();
        Human!();
        Alien!();
        Object!();
        Intelligent!();
        Query!();
        CatOrDog!();
        OneofArg!();
        Being!();
        HumanOrAlien!();
        DogOrHuman!();
        Cat!();
        ID!();
        ComplicatedArgs!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        # [Object (internal)] impl Query { async fn human (& self , id : Option < ID >) -> Option < Human > { unimplemented ! () } async fn alien (& self) -> Option < Alien > { unimplemented ! () } async fn dog (& self) -> Option < Dog > { unimplemented ! () } async fn cat (& self) -> Option < Cat > { unimplemented ! () } async fn pet (& self) -> Option < Pet > { unimplemented ! () } async fn being (& self) -> Option < Being > { unimplemented ! () } async fn intelligent (& self) -> Option < Intelligent > { unimplemented ! () } async fn cat_or_dog (& self) -> Option < CatOrDog > { unimplemented ! () } async fn dog_or_human (& self) -> Option < DogOrHuman > { unimplemented ! () } async fn human_or_alien (& self) -> Option < HumanOrAlien > { unimplemented ! () } async fn complicated_args (& self) -> Option < ComplicatedArgs > { unimplemented ! () } async fn oneof_arg (& self , arg : OneofArg) -> String { unimplemented ! () } async fn oneof_opt (& self , arg : Option < OneofArg >) -> String { unimplemented ! () } }
    };
}

impl_163!();