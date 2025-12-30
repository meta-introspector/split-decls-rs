// Generated macro for RuleState (enum)
macro_rules! Depcrate_lineRuleState {
() => {
// Module: crate::line
// Provides: {"RuleState"}
// Dependencies: {}
enum RuleState < 'a > { Start , Name , FromYear { name : & 'a str , } , ToYear { name : & 'a str , from_year : Year , } , Type { name : & 'a str , from_year : Year , to_year : Option < Year > , } , Month { name : & 'a str , from_year : Year , to_year : Option < Year > , } , Day { name : & 'a str , from_year : Year , to_year : Option < Year > , month : Month , } , Time { name : & 'a str , from_year : Year , to_year : Option < Year > , month : Month , day : DaySpec , } , TimeToAdd { name : & 'a str , from_year : Year , to_year : Option < Year > , month : Month , day : DaySpec , time : TimeSpecAndType , } , Letters { name : & 'a str , from_year : Year , to_year : Option < Year > , month : Month , day : DaySpec , time : TimeSpecAndType , time_to_add : TimeSpec , } , }
};
}
