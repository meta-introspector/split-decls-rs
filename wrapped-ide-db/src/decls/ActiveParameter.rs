macro_rules! ActiveParameter {
    () => {
        # [derive (Debug)] pub struct ActiveParameter < 'db > { pub ty : Type < 'db > , pub src : Option < InFile < Either < ast :: SelfParam , ast :: Param > > > , }
    };
}

ActiveParameter!()