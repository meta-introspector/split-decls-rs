macro_rules! deps {
    () => {
        RegionName!();
    };
}

macro_rules! SuggestedConstraint {
    () => {
        deps!();
        # [doc = " The different things we could suggest."] enum SuggestedConstraint { # [doc = " Outlives(a, [b, c, d, ...]) => 'a: 'b + 'c + 'd + ..."] Outlives (RegionName , SmallVec < [RegionName ; 2] >) , # [doc = " 'a = 'b"] Equal (RegionName , RegionName) , # [doc = " 'a: 'static i.e. 'a = 'static and the user should just use 'static"] Static (RegionName) , }
    };
}

SuggestedConstraint!();