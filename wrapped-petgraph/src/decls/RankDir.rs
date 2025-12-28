macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! RankDir {
    () => {
        deps!();
        # [doc = " Direction of graph layout."] # [doc = ""] # [doc = " <https://graphviz.org/docs/attrs/rankdir/>"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum RankDir { # [doc = " Top to bottom"] TB , # [doc = " Bottom to top"] BT , # [doc = " Left to right"] LR , # [doc = " Right to left"] RL , }
    };
}

RankDir!();