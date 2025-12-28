macro_rules! TOKEN_UPGRADABLE {
    () => {
        const TOKEN_UPGRADABLE : ParkToken = ParkToken (ONE_READER | UPGRADABLE_BIT) ;
    };
}

TOKEN_UPGRADABLE!();